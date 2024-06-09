import type { LinkExpression, PerspectiveDiff, PerspectiveExpression, PerspectiveState } from '@coasys/ad4m';

export interface Triple {
    source?: string;
    target?: string;
    predicate?: string;
}

export interface ExpressionProof {
    key: string;
    signature: string;
}

export interface Perspective {
    links: LinkExpression[];
}

declare global {
    interface RustLanguages {
        perspectiveDiffReceived: (diff: PerspectiveDiff, languageAddress: string) => void;
        syncStateChanged: (state: PerspectiveState, languageAddress: string) => void;
        telepresenceSignalReceived: (signal: PerspectiveExpression, languageAddress: string) => void;
    }

    const LANGUAGE_CONTROLLER: RustLanguages;
}
