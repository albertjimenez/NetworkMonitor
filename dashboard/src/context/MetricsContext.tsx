import React, {createContext, ReactNode, useReducer} from 'react';
import Metrics from "../interfaces/Metrics.ts";
import {useNatsSubscription} from "../hooks/useNats.tsx";
import {JSONCodec, Msg} from "nats.ws";

// Define initial state and reducer
interface MetricsState {
    metrics: Metrics | undefined;
    lastUpdated: number | null;
    loading: boolean;
}

interface ActionProps {
    type: "SET_METRICS" | "SET_LOADING",
    payload?: Metrics
}

const initialState: MetricsState = {
    metrics: undefined,
    lastUpdated: null,
    loading: true,
};
const natsSubject = 'system_monitor.data';


const metricsReducer = (state: MetricsState, action: ActionProps): MetricsState => {
    switch (action.type) {
        case 'SET_METRICS':
            return {
                ...state,
                metrics: action.payload,
                lastUpdated: Date.now(),
                loading: false,
            };
        case 'SET_LOADING':
            return {
                ...state,
                loading: true,
            };
        default:
            return state;
    }
};

// Create context
export const MetricsContext = createContext<{
    state: MetricsState;
    dispatch: React.Dispatch<ActionProps>;
} | null>(null);

interface MetricsProviderProps {
    children: ReactNode;  // Add this type for children
}

export const MetricsProvider: React.FC<MetricsProviderProps> = ({children}) => {
    const [state, dispatch] = useReducer(metricsReducer, initialState);
    const sc = JSONCodec();


    useNatsSubscription(natsSubject, async (msg: Msg) => {
        const data = sc.decode(msg.data) as Metrics;
        dispatch({type: "SET_METRICS", payload: data});
    });

    return (
        <MetricsContext.Provider value={{state, dispatch}}>
            {children}
        </MetricsContext.Provider>
    );
};

// Hook to access the MetricsContext

