import * as lc from 'vscode-languageclient';

export class Server {
    static connect() {
        const run: lc.Executable = {
            command: 'secsp_language_server',
            options: { cwd: '.' }
        };

        const debug = {
            args: [
                "--log-level", "debug"
            ],
            ...run
        };

        const serverOptions: lc.ServerOptions = { run, debug };
        const clientOptions: lc.LanguageClientOptions = {
            documentSelector: [{ scheme: 'file', language: 'secsp' }],
            initializationOptions: {
                publishDecorations: true
            }
        }

        const client = new lc.LanguageClient('secsp-lang-server', serverOptions, clientOptions, false);
        client.start();
    }
}