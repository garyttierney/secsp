export const ANALYSIS_INIT = "init";
export const CREATE_CODE_BLOCK = "create-code-block";
export const UPDATE_CODE_BLOCK = "update-code-block";

export type AnalysisResponseTypeOf<P> = P extends RequestType<infer T>
    ? T
    : any;
export type AnalysisRequestType =
    | CreateCodeBlockRequest
    | UpdateCodeBlockRequest
    | InitializeAnalysisRequest;

interface RequestType<ResponseType> {
    type: string;
    _responseSentinel?: ResponseType | null;
}

export interface InitializeAnalysisRequest extends RequestType<boolean> {
    type: typeof ANALYSIS_INIT;
}

export interface CreateCodeBlockRequest
    extends RequestType<CreateCodeBlockResponse> {
    type: typeof CREATE_CODE_BLOCK;
    code: string;
}

export interface CreateCodeBlockResponse {
    id: string;
}

export interface UpdateCodeBlockRequest
    extends RequestType<UpdateCodeBlockResponse> {
    type: typeof UPDATE_CODE_BLOCK;
    id: string;
    code: string;
}

export interface UpdateCodeBlockResponse {
    successful: boolean;
}

export class RequestIdGenerator {
    modulo: number;
    half: number;
    value: number;

    constructor(idBits: number) {
        this.modulo = Math.pow(2, idBits);
        this.half = Math.pow(2, idBits - 1);
        this.value = 0;
    }

    next(): number {
        return this.value++ % this.modulo;
    }
}
