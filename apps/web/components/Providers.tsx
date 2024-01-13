'use client';

import { client, queryClient, rspc } from "../src/lib/rspc";
import { PropsWithChildren } from "react";

export function Providers({ children }: PropsWithChildren<{ }>) {
    return (
        <rspc.Provider client={client} queryClient={queryClient}>
            { children as any }
        </rspc.Provider>
    );
}
