import {connect, Msg, NatsConnection, NatsError} from 'nats.ws'
import {useEffect, useState} from 'react';


const NATS_URL = import.meta.env.VITE_NATS_URL;
export const natsClientPromise = connect({servers: NATS_URL})

const _onError = (error: unknown) => {
    console.log(error);
}

export function useNats(onSuccess: () => void = () => {
}, onError: (error: unknown) => void = (error: unknown) => _onError(error)) {
    const [nats, setNats] = useState<NatsConnection | null>(null)

    useEffect(() => {
        if (!nats) {
            natsClientPromise
                .then((nc) => {
                    setNats(nc);
                    onSuccess();
                })
                .catch(onError)
        }
    }, [])

    return nats
}

type SuccessCallback = (msg: Msg) => Promise<void>

export function useNatsSubscription(subj: string, onMessage: SuccessCallback) {
    const nc = useNats()
    useEffect(() => {
        if (!nc) return
        const sub = nc.subscribe(subj, {
            callback: function (err: NatsError | null, msg: Msg) {
                if (err) {
                    console.error(err)
                } else {
                    onMessage(msg)
                }
            },
        })
        return () => {
            sub.unsubscribe()
        }
    }, [nc])
}
