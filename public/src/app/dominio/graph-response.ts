import { graphFromBack } from "./graph-from-back";

export interface graphResponse {
    data: graphFromBack,
    duration: number,
    status: string
}