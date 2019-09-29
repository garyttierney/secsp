import {
    ANALYSIS_INIT,
    AnalysisRequestType,
    AnalysisResponseTypeOf,
    CreateCodeBlockRequest,
    UPDATE_CODE_BLOCK,
    UpdateCodeBlockRequest,
    RequestIdGenerator,
    CREATE_CODE_BLOCK,
    InitializeAnalysisRequest
} from "../common/analysis-protocol";

export default class AnalysisHost {
    private rejectPool: Map<number, (reason?: any) => void> = new Map();
    private resolvePool: Map<number, (value?: unknown) => void> = new Map();

    private requestIdGenerator = new RequestIdGenerator(32);
    private worker: Worker;

    constructor(worker: Worker) {
        this.worker = worker;
        this.worker.onmessage = async m => this.handle(m);
    }

    protected async handle(msg: MessageEvent) {
        const { id, err, payload } = msg.data;

        if (payload) {
            this.resolvePool[id](payload);
        } else {
            this.rejectPool[id](err);
        }

        delete this.resolvePool[id];
        delete this.rejectPool[id];
    }

    protected send<T extends AnalysisRequestType>(
        request: T
    ): Promise<AnalysisResponseTypeOf<T>> {
        const requestId = this.requestIdGenerator.next();

        const promise = new Promise((resolve, reject) => {
            this.resolvePool[requestId] = resolve;
            this.rejectPool[requestId] = reject;

            this.worker.postMessage({
                id: requestId,
                type: typeof request,
                request: request
            });
        });

        return promise as Promise<AnalysisResponseTypeOf<T>>;
    }

    public async initialize(): Promise<boolean> {
        return await this.send<InitializeAnalysisRequest>({
            type: ANALYSIS_INIT
        });
    }

    public async createCodeBlock(code: string): Promise<string> {
        const { id } = await this.send<CreateCodeBlockRequest>({
            type: CREATE_CODE_BLOCK,
            code
        });

        return id;
    }

    public async updateCodeBlock(id: string, code: string): Promise<boolean> {
        const { successful } = await this.send<UpdateCodeBlockRequest>({
            type: UPDATE_CODE_BLOCK,
            id,
            code
        });

        return successful;
    }
}
