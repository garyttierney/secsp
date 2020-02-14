import * as monaco from 'monaco-editor/esm/vs/editor/editor.api';

import AnalysisClient from './editor/analysis-client';
import * as secsp from './editor/config/secsp-language';
import Worker from './worker';

document.addEventListener('DOMContentLoaded', async () => {
    const editorElements = Array.from(document.querySelectorAll('#try, .language-csp'));

    if (editorElements.length == 0) {
        console.info("No editor elements found, not initializing analysis host");
        return;
    }

    const client = new AnalysisClient(new Worker);
    const initialized = await client.initialize();

    if (!initialized) {
        console.error("Unable to initialize analysis client");
    }

    const editorSettings: monaco.editor.IEditorConstructionOptions  = {
        theme: 'vs-dark',
        language: 'secsp',
        minimap: {
            enabled: false,
        },
        scrollbar: {
            // Don't interfere with regular browser scrolling.
            handleMouseWheel: false
        }
    };

    monaco.languages.register({id: secsp.id, extensions: ['.csp']});
    monaco.languages.setLanguageConfiguration(secsp.id, secsp.languageConfiguration);
    monaco.languages.setMonarchTokensProvider(secsp.id, secsp.language);

    const replaceElementWithEditor = (el: HTMLElement) => {
        if (el.tagName === 'CODE') {
            const parent = el.parentElement;
            const code = el.innerText;
            const {clientWidth, clientHeight} = el;

            const newElement = document.createElement("div");
            parent.replaceWith(newElement);

            const editor = monaco.editor.create(newElement, editorSettings);
            editor.layout({height: clientHeight, width: clientWidth});

            return {
                code,
                editor
            };
        } else {
            const code = el.innerText;
            const editor = monaco.editor.create(el, editorSettings);
            editor.layout({height: el.clientHeight, width: el.clientWidth});

            return {
                code,
                editor
            };
        }
    };

    for (let element of editorElements) {
        const {code, editor} = replaceElementWithEditor(element as HTMLElement);
        const id = await client.createCodeBlock(code);

        editor.setValue(code);
        editor.onDidChangeModelContent(async (e) => {
            const model = editor.getModel();
            const text = model.getValue();

            await client.updateCodeBlock(id, text);
        });
    }
});
