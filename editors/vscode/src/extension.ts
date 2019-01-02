import { ExtensionContext } from 'vscode';
import { Server } from './server';

export function activate(context: ExtensionContext) {
    Server.connect();
}