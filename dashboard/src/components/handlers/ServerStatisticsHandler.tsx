import {useEffect} from "react";
import ServerStatistics from "../ServerStatistics.tsx";
import {useMetrics} from "../../hooks/useMetrics.tsx";


export default function ServerStatisticsHandler() {
    const {state, dispatch} = useMetrics();

    useEffect(() => {
        if (!state.metrics) {
            dispatch({type: 'SET_LOADING'});
        }
    }, [state.metrics]);

    return <ServerStatistics metrics={state.metrics} isLoading={state.loading}/>;
}
