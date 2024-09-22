import React from "react";
import {TemperatureInfo} from "../../interfaces/Metrics.ts";
import {Card, Col, Progress, Row, Typography} from "antd";
import {red} from "@ant-design/colors";
import { Empty } from 'antd';

interface SingleTemperatureWidgetProps {
    temperatureObject?: TemperatureInfo
}

interface TemperatureWidgetsProps {
    temperatures: TemperatureInfo[]
}

const TemperatureWidget: React.FC<SingleTemperatureWidgetProps> = (props: SingleTemperatureWidgetProps) => {
    const temperature = props?.temperatureObject || {name: "", temperature: 0};
    return <Col span={12}>
        <Card bordered={true} title={temperature.name}>
            <Progress type="line" format={(percent) => `${percent}ºC`} status={"active"} strokeColor={red[5]} percent={+temperature.temperature.toFixed(2)}/>
        </Card>
    </Col>

}
const TemperatureWidgets: React.FC<TemperatureWidgetsProps> = (props: TemperatureWidgetsProps) => {
    const temperatures = props?.temperatures || [];
    if (temperatures.length === 0) {
        return <Empty description={<Typography.Text>
            Not detected sensor temperatures. (Docker has no access to them)
        </Typography.Text>}/>
    }
    const mappedTemperatures = temperatures.map((temperature?: TemperatureInfo) => {
        return <TemperatureWidget temperatureObject={temperature}/>
    });
    return <Row gutter={24}>
        {mappedTemperatures}
    </Row>
}
export default TemperatureWidgets;
