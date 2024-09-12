import React, {useEffect, useState} from 'react';
import {Layout, Menu, message} from 'antd';
import {DashboardOutlined, LinkOutlined, MonitorOutlined, SettingOutlined} from '@ant-design/icons';
import "./App.css";
import NetworkMap from "./components/NetworkMap.tsx";
import Header from "./components/Header.tsx";
import DeviceTableHandler from "./components/handlers/DeviceTableHandler.tsx";
import ServerStatisticsHandler from "./components/handlers/ServerStatisticsHandler.tsx";
import {Link, Route, Routes, useLocation} from "react-router-dom";
import Settings from "./components/Settings.tsx";
import LogEvents from "./components/LogEvents.tsx";
import {useNats} from "./hooks/useNats.tsx";

const {Sider, Content} = Layout;

enum RoutesNameMenu {
    "/" = "1",
    "/metrics" = "2",
    "/logs" = "3",
    "/settings" = "4",
}

const App: React.FC = () => {
    const onNatsConnectionError = (error: unknown) => {
        console.log(error);
        message.error("Connection to NATS Queue cannot be established, check the NATS_URL environment variable.")
    }
    const [menuKey, setMenuKey] = useState<string>("1");
    const location = useLocation().pathname;
    useEffect(() => {
        const currentLocation = location.toLowerCase();
        if (currentLocation in RoutesNameMenu)
            setMenuKey(RoutesNameMenu[currentLocation as keyof typeof RoutesNameMenu].toString());
        else
            setMenuKey(RoutesNameMenu["/"]);
    }, [location]);

    useNats(() => {
    }, onNatsConnectionError)
    const siderMenuItems = [
        {
            key: '1',
            icon: <LinkOutlined/>,
            label: <Link to="/">Devices</Link>,
        },
        {
            key: '2',
            icon: <DashboardOutlined/>,
            label: <Link to="/metrics">Metrics</Link>,
        },
        {
            key: '3',
            icon: <MonitorOutlined/>,
            label: <Link to="/logs">Logs</Link>,
        },
        {
            key: '4',
            icon: <SettingOutlined/>,
            label: <Link to="/settings">Settings</Link>,
        },
    ];


    return (
        <Layout style={{minHeight: '100vh'}}>
            <Header/>
            <br/>
            <Layout>
                <Sider
                    width={200}
                    className="site-layout-background"
                    style={{display: 'block'}} // Ensure Sider is visible on larger screens
                    collapsible
                    breakpoint="lg" // Sider will be hidden on small screens
                    collapsedWidth="0" // Set collapsed width to 0 to hide on smaller screens
                >
                    <Menu mode="inline" selectedKeys={[menuKey]}
                          style={{height: '100%', borderRight: 0}}
                          items={siderMenuItems}/>

                </Sider>
                <Layout style={{padding: '0 24px', minHeight: 'calc(100vh - 64px)'}}>
                    <Content style={{padding: 24, margin: 0, minHeight: 280}}>
                        <Routes>
                            <Route path="/network" element={<NetworkMap/>}/>
                            <Route path="/metrics" element={<ServerStatisticsHandler/>}/>
                            <Route path="/settings" element={<Settings/>}/>
                            <Route path="/logs" element={<LogEvents/>}/>
                            <Route path="/" element={<DeviceTableHandler></DeviceTableHandler>}/>
                        </Routes>
                    </Content>
                </Layout>
            </Layout>
        </Layout>
    );

};

export default App;
