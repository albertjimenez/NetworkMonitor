import {useContext} from "react";
import {MetricsContext} from "../context/MetricsContext.tsx";

export const useMetrics = () => {
    const context = useContext(MetricsContext);
    if (!context) {
        throw new Error('useMetrics must be used within a MetricsProvider');
    }
    return context;
};
