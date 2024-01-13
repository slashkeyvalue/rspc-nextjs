import { createClient, FetchTransport } from "@rspc/client";
import { createReactQueryHooks } from "@rspc/react";
import { QueryClient } from "@tanstack/react-query";

import type { Procedures } from '../utils/bindings.js';
import { PropsWithChildren } from "react";

const rspcClient = createClient<Procedures>({
    transport: new FetchTransport('http://localhost:4000/rspc')
});

const queryClient = new QueryClient();

export const rspc = createReactQueryHooks<Procedures>();

export function RspcProvider({ children }: PropsWithChildren<{ }>)
{
    return (
        <rspc.Provider client={rspcClient} queryClient={queryClient}>
            { children as any }
        </rspc.Provider>
    )
}
