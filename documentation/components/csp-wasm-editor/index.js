import React from 'react';
import ReactDOM from 'react-dom';
import MonacoEditor from 'react-monaco-editor';

import * as monaco from 'monaco-editor/esm/vs/editor/editor.api';
import 'monaco-editor/esm/vs/editor/contrib/contextmenu/contextmenu';
import 'monaco-editor/esm/vs/editor/standalone/browser/quickOpen/gotoLine';
import 'monaco-editor/esm/vs/editor/standalone/browser/quickOpen/quickCommand';
import 'monaco-editor/esm/vs/editor/standalone/browser/quickOpen/quickOutline';
import 'babel-polyfill';

const LANG_ID = 'secsp';

monaco.languages.register({id: LANG_ID,});
monaco.languages.setLanguageConfiguration(LANG_ID, {
    comments: {
        lineComment: "//",
        blockComment: ["/*", "*/"]
    },
    brackets: [
        ['{', '}'],
        ['[', ']'],
        ['(', ')'],
    ],
});
monaco.languages.setMonarchTokensProvider(LANG_ID, {
    keywords: [
        'abstract', 'block', 'optional',
        'if', 'else',
        'true', 'false',
        'allow', 'never_allow', 'audit_allow', 'dont_audit'
    ],
    typeKeywords: [
        'type', 'type_attribute', 'type_alias',
        'role', 'role_attribute',
        'user', 'user_attribute'
    ],

    operators: [
        '=', '>', '<', '!', '~', '?', ':',
        '==', '<=', '>=', '!=', '&&', '||', '++', '--',
        '+', '-', '*', '/', '&', '|', '^', '%', '<<',
        '>>', '>>>', '+=', '-=', '*=', '/=', '&=', '|=',
        '^=', '%=', '<<=', '>>=', '>>>='
    ],
    symbols: /[=><!~?:&|+\-*\/\^%]+/,
    tokenizer: {
        root: [
            [/[a-z_$][\w$]*/, {
                cases: {
                    '@keywords': 'keyword',
                    '@typeKeywords': 'type',
                    '@default': 'identifier'
                }
            }],
            {include: '@whitespace'},

            [/[{}()\[\]]/, '@brackets'],
            [/[<>](?!@symbols)/, '@brackets'],
            [/@symbols/, {
                cases: {
                    '@operators': 'delimiter',
                    '@default': ''
                }
            }],
        ],

        comment: [
            [/[^\/*]+/, 'comment'],
            [/\/\*/, 'comment', '@push'],
            ["\\*/", 'comment', '@pop'],
            [/[\/*]/, 'comment']
        ],

        whitespace: [
            [/[ \t\r\n]+/, 'white'],
            [/\/\*/, 'comment', '@comment'],
            [/\/\/.*$/, 'comment'],
        ],
    }
});

class CspEditor extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            code: `// type your code.
type t;
allow t self : process read;`,
            language: 'text'
        }
    }

    async componentDidMount() {
        try {
            const rustapi = await import("./pkg");
            rustapi.start();

            this.analysis = new rustapi.SingleFileAnalysis();
        } catch (err) {
            console.error("Unable to load analysis API", err);
        }
    }

    onChange(text) {
        if (this.analysis != null) {
            this.analysis.update(text);
        }

        this.setState((state) => {
            return {...state, 'code': text};
        });
    }

    render() {
        const {code} = this.state;

        let options = {
            selectOnLineNumbers: true,
            lineNumbers: 'on',
            contextmenuenu: true,
            renderMinimap: false,
        };

        let monacoEditor = <MonacoEditor
            height="250"
            language="secsp"
            theme="vs-dark"
            value={code}
            options={options}
            onChange={::this.onChange}
        />;

        return (
            monacoEditor
        );
    }
}


document.addEventListener('DOMContentLoaded', () => {
    let editor = <CspEditor/>;
    ReactDOM.render(editor, document.getElementById('try'));

});
