import {NatsConnection, StringCodec} from "nats.ws";
import {Constants} from "../data/Constants.ts";

export async function sendToKV(nc: NatsConnection, key: string, value: string) {
    try {
        const sc = StringCodec();
        const js = nc.jetstream();
        const kv = await js.views.kv("settings");

        const e = await kv.get(key);
        if (!e) {
            await kv.create(key, sc.encode(Constants.DEFAULT_TIMEOUT.toString()));
            console.log(`Key created with value ${Constants.DEFAULT_TIMEOUT}`);
        } else {
            await kv.put(key, sc.encode(value));
            console.log(`Key ${key} updated to ${value}, previous value was: ${sc.decode(e.value)}`);
        }

        await nc.flush();
    } catch (err) {
        console.error("Error interacting with KV store:", err);
    }
}
