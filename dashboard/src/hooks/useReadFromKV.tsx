import { useState, useEffect } from 'react';
import {StringCodec} from "nats.ws";
import {useNats} from "./useNats.tsx";


const useReadFromKV = ( key: string) => {
    const [value, setValue] = useState<number | null>(null);
    const [error, setError] = useState<string | null>(null);
    const nc = useNats();
    useEffect(() => {
        const readFromKV = async () => {
            try {
                const sc = StringCodec(); // Assuming this is defined elsewhere in your app
                const js = nc?.jetstream();
                const kv = await js?.views.kv("settings");

                const entry = await kv?.get(key);
                if (entry) {
                    const decodedValue = +sc.decode(entry.value);
                    setValue(decodedValue);
                }
            } catch (err) {
                console.error("Error reading from KV store:", err);
                setError(`Error reading from KV store: ${err}`);
            }
        };

        readFromKV();
    }, [nc, key]); // Re-run effect when nc or key changes

    return { value, error };
};

export default useReadFromKV;
