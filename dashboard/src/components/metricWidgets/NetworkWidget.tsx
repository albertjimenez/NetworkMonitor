import {Card, Col, Row, Statistic} from "antd";
import {ArrowDownOutlined, ArrowUpOutlined} from "@ant-design/icons";
import React from "react";
import {NetworkUsage} from "../../interfaces/Metrics.ts";
import UnitConverter from "../../utils/UnitConverter.ts";

interface NetworkWidgetProps {
    network_usage?: NetworkUsage
}

const NetworkWidget: React.FC<NetworkWidgetProps> = (props) => {
    const {rx, tx, name: interfaceName} = props?.network_usage || {rx: 1, tx: 1, name: ""};
    const formattedRx = UnitConverter.bitsToHumanReadable(rx);
    const formattedTx = UnitConverter.bitsToHumanReadable(tx);
    return <Row gutter={24}>
        <Col span={12}>
            <Card bordered={true} title={`Interface: ${interfaceName}`}>
                <Statistic
                    title={`Download`}
                    value={formattedRx.value}
                    precision={2}
                    valueStyle={{color: '#3f8600'}}
                    prefix={<ArrowUpOutlined/>}
                    suffix={formattedRx.label}
                />
            </Card>
        </Col>
        <Col span={12}>
            <Card bordered={true} title={`Interface: ${interfaceName}`}>
                <Statistic
                    title={`Upload`}
                    value={formattedTx.value}
                    precision={2}
                    valueStyle={{color: '#134fcf'}}
                    prefix={<ArrowDownOutlined/>}
                    suffix={formattedTx.label}
                />
            </Card>
        </Col>
    </Row>
}
export default NetworkWidget;
