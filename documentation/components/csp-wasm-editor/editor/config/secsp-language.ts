import * as monaco from 'monaco-editor/esm/vs/editor/editor.api';
import {languages} from 'monaco-editor/esm/vs/editor/editor.api';
import IMonarchLanguage = languages.IMonarchLanguage;

export const id = 'secsp';
export const languageConfiguration: monaco.languages.LanguageConfiguration = {
    comments: {
        lineComment: "//",
        blockComment: ["/*", "*/"]
    },
    brackets: [
        ['{', '}'],
        ['[', ']'],
        ['(', ')'],
    ],
};
export const language = <IMonarchLanguage> {
    keywords: [
        'abstract', 'block', 'optional',
        'if', 'else',
        'true', 'false',
        'allow', 'never_allow', 'audit_allow', 'dont_audit',
        'macro',
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
                    '@default': 'variable'
                }
            }],
            {include: '@whitespace'},

            [/[{}()\[\]]/, '@brackets'],
            [/[<>](?!@symbols)/, '@brackets'],
            [/@symbols/, {
                cases: {
                    '@operators': 'operator.misc',
                    '@default': ''
                }
            }],
        ],

        comment: [
            [/[^\/*]+/, 'comment'],
            [/\/\*/, 'comment', '@push'],
            [/\\*/, 'comment', '@pop'],
            [/[\/*]/, 'comment']
        ],

        whitespace: [
            [/[ \t\r\n]+/, 'white'],
            [/\/\*/, 'comment', '@comment'],
            [/\/\/.*$/, 'comment'],
        ],
    }
};
