import React, {useEffect} from "react";
import {useNats} from "../hooks/useNats.tsx";
import useReadFromKV from "../hooks/useReadFromKV.tsx";
import {Button, Divider, Form, Input, message, Typography} from "antd";
import {sendToKV} from "../hooks/sendToKV.tsx";
import {Constants} from "../data/Constants.ts";

const {Title} = Typography;

interface FormData {
    refreshTime: string,
    hostInterval: string
}

const Settings: React.FC = () => {
    const nc = useNats();

    const kv_refresh_time = useReadFromKV(Constants.SYSTEM_MONITOR_REFRESH_TIME).value;
    const kv_host_scan_interval = useReadFromKV(Constants.HOST_SCAN_INTERVAL_TIME).value;

    const [form] = Form.useForm();

    const onFinish = async (values: FormData) => {
        if (nc != null) {
            form.setFieldsValue({
                refreshTime: values.refreshTime,
                hostInterval: values.hostInterval,
            });
            await sendToKV(nc, Constants.SYSTEM_MONITOR_REFRESH_TIME, values.refreshTime);
            await sendToKV(nc, Constants.HOST_SCAN_INTERVAL_TIME, values.hostInterval);
            await message.success("Values updated successfully!");
        }
    };
    useEffect(() => {
        form.setFieldsValue({
            refreshTime: kv_refresh_time,
            hostInterval: kv_host_scan_interval,
        });
    }, [form, nc, kv_refresh_time, kv_host_scan_interval]);
    return <>
        <Title level={2}>Settings</Title>
        <Divider/>
        <Title level={3}>System Monitor</Title>
        <Form
            layout={"vertical"}
            form={form}
            onFinish={onFinish}>
            <Form.Item
                label="System Monitor Refresh Time"
                name="refreshTime"
                rules={[{required: true, message: "Please input a refresh time!"}]}
            >
                <Input type="number" placeholder="Enter refresh time in seconds"/>
            </Form.Item>
            <Divider/>
            <Title level={3}>Host Scan</Title>
            <Form.Item
                label="Host Scan Interval"
                name="hostInterval"
                rules={[{required: true, message: "Please input a host scan interval!"}]}
            >
                <Input type="number" placeholder="Enter host scan interval in seconds"/>
            </Form.Item>

            <Form.Item>
                <Button type="primary" htmlType="submit">
                    Submit
                </Button>
            </Form.Item>
        </Form>
    </>
}

export default Settings;
