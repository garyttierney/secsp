import {
    ANALYSIS_INIT,
    AnalysisRequestType,
    AnalysisResponseTypeOf,
    CREATE_CODE_BLOCK,
    CreateCodeBlockRequest,
    CreateCodeBlockResponse,
    UPDATE_CODE_BLOCK,
    UpdateCodeBlockRequest,
    UpdateCodeBlockResponse
} from "../common/analysis-protocol";

declare type SingleFileAnalysis = import("../pkg").SingleFileAnalysis;

/**
 * The wasm-pack tool generates a JavaScript file that exports a function-like
 * object containing the analysis API. When invoked, the object will load the name
 * of the WASM bundle given.
 *
 * Otherwise, the API behaves exactly like the typings say.
 */
declare type rustapi = typeof import("../pkg") & ((string) => Promise<void>);

export class AnalysisHost {
    private analysis: SingleFileAnalysis;
    private api: rustapi;
    private fileCounter: number = 0;

    constructor(api: rustapi) {
        this.api = api;
    }

    public async handle(
        request: AnalysisRequestType
    ): Promise<AnalysisResponseTypeOf<AnalysisRequestType>> {
        switch (request.type) {
            case ANALYSIS_INIT: {
                return this.initializeAnalysis();
            }
            case CREATE_CODE_BLOCK: {
                return this.createCodeBlock(request);
            }
            case UPDATE_CODE_BLOCK: {
                return this.updateCodeBlock(request);
            }
        }
    }

    protected async initializeAnalysis(): Promise<boolean> {
        await this.api("index_bg.wasm");

        this.api.start();
        this.analysis = new this.api.SingleFileAnalysis();

        return true;
    }

    protected createCodeBlock(
        request: CreateCodeBlockRequest
    ): CreateCodeBlockResponse {
        let id = `fragment-${this.fileCounter++}.csp`;
        this.analysis.create_file(id, request.code);

        return {
            id
        };
    }

    protected updateCodeBlock(
        request: UpdateCodeBlockRequest
    ): UpdateCodeBlockResponse {
        const { id, code } = request;
        this.analysis.update(id, code);

        return { successful: true };
    }
}
