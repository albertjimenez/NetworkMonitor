import React from "react";
import {TemperatureInfo} from "../../interfaces/Metrics.ts";
import {Card, Col, Progress, Row} from "antd";
import {red} from "@ant-design/colors";

interface TemperatureWidgetProps {
    temperatureObject?: TemperatureInfo
}

interface TemperatureWidgetsProps {
    temperatures: TemperatureInfo[]
}

const TemperatureWidget: React.FC<TemperatureWidgetProps> = (props: TemperatureWidgetProps) => {
    const temperature = props?.temperatureObject || {name: "", temperature: 0};
    return <Col span={12}>
        <Card bordered={true} title={temperature.name}>
            <Progress type="line" format={(percent) => `${percent}ºC`} status={"active"} strokeColor={red[5]} percent={+temperature.temperature.toFixed(2)}/>
        </Card>
    </Col>

}
const TemperatureWidgets: React.FC<TemperatureWidgetsProps> = (props: TemperatureWidgetsProps) => {
    const temperatures = props?.temperatures || [];
    const mappedTemperatures = temperatures.map((temperature?: TemperatureInfo) => {
        return <TemperatureWidget temperatureObject={temperature}/>
    });
    return <Row gutter={24}>
        {mappedTemperatures}
    </Row>
}
export default TemperatureWidgets;
