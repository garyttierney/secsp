import monaco from 'monaco-editor/esm/vs/editor/editor.api';
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

document.addEventListener('DOMContentLoaded', async () => {
    let analysis;
    let rustapi;
    try {
        rustapi = await import("./pkg");
        rustapi.start();

        analysis = new rustapi.SingleFileAnalysis();
    } catch (err) {
        console.error("Unable to load analysis API", err);
    }

    const editorElement = document.getElementById('try');

    const exampleCode = `// type your code.
type t;
allow t self : process read;`;

    const editor = monaco.editor.create(editorElement, {
        theme: 'vs-dark',
        value: exampleCode,
        language: 'secsp',
        minimap: {
            enabled: false,
        },
    });

    editor.layout({height: 250, width: editorElement.clientWidth});

    editor.onDidChangeModelContent((e) => {
        const model = editor.getModel();
        const text = model.getValue();

        analysis.update(text);
    })
});
