import {Device} from "./Device.ts";

export interface DeviceWithKey extends Device{
    key: string,
}
