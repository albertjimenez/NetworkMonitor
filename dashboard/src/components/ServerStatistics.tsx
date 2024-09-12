import React from 'react';
import {green, orange, red} from '@ant-design/colors';

import {Card, Col, Divider, Progress, Row, Skeleton, Typography} from 'antd';
import "../styles/RouterStatisticsCSS.css";
import Metrics from "../interfaces/Metrics.ts";
import UnitConverter from "../utils/UnitConverter.ts";
import TemperatureWidgets from "./metricWidgets/TemperatureWidgets.tsx";
import NetworkWidget from "./metricWidgets/NetworkWidget.tsx";

const {Title} = Typography;


interface ServerStatisticsProps {
    metrics?: Metrics,
    isLoading: boolean
}

const ServerStatistics: React.FC<ServerStatisticsProps> = (props: ServerStatisticsProps) => {
    const {metrics, isLoading} = props;
    const diskTotal = UnitConverter.bytesToHumanReadable((metrics?.disk_usage?.total) || 1.0);
    const diskAvailable = UnitConverter.bytesToHumanReadable((metrics?.disk_usage?.available) || 1.0);

    const memoryUsed = UnitConverter.bytesToHumanReadable(metrics?.mem_usage?.used || 1);
    const memoryTotal = UnitConverter.bytesToHumanReadable(metrics?.mem_usage?.total || 1);


    return (
        <Card title="Server Monitoring" bordered>
            <Skeleton loading={isLoading}>

                <Title level={4}>Hardware Usages</Title>

                <Row gutter={24}>
                    <Col span={8}>
                        <Card bordered={true} title="CPU Usage">
                            <Progress type="circle" percent={metrics?.cpu_usage?.usage}/>
                        </Card>
                    </Col>
                    <Col span={8}>
                        <Card bordered={true} title={`Disk Available - Total ${diskTotal.value} ${diskTotal?.label}`}>
                            <Progress percent={diskAvailable.value} steps={5} type={"circle"}
                                      format={(percent) => `${percent} ${diskAvailable.label}`}
                                      strokeColor={[green[6], green[6], orange[5], red[5], red[6]]}/>
                        </Card>
                    </Col>
                    <Col span={8}>
                        <Card bordered={true} title={`Mem Used - Total ${memoryTotal.value} ${memoryTotal?.label}`}>
                            <Progress percent={memoryUsed.value} steps={5}
                                      type={"circle"}
                                      format={percent => `${percent} ${memoryUsed.label}`}
                                      strokeColor={[green[6], green[6], orange[5], red[5], red[6]]}/>
                        </Card>
                    </Col>
                </Row>

                <Divider/>
                <Title level={4}>Network Usage</Title>

                <NetworkWidget network_usage={metrics?.network_usage}/>
                <Divider/>

                <Title level={4}>Temperatures</Title>

                <TemperatureWidgets temperatures={metrics?.temperature_info || []}/>
            </Skeleton>
        </Card>
    );
};

export default ServerStatistics;
