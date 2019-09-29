import { AnalysisRequestType } from './common/analysis-protocol';
import { AnalysisHost } from './worker/analysis-host';

// https://github.com/Microsoft/TypeScript/issues/14877#issuecomment-493729050
export default null;
declare var self: WorkerGlobalScope & Worker;
declare var wasm_bindgen: typeof import('./pkg');

self.importScripts('/js/csp-wasm-pkg.js');

// @ts-ignore
let host = new AnalysisHost(wasm_bindgen);

self.onmessage = async (message) => {
    const id : number = message.data.id;
    const request : AnalysisRequestType = message.data.request;
    const response = await host.handle(request);

    self.postMessage({
        id,
        payload: response
    });
};

