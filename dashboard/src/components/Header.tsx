import {Button, Layout} from 'antd';
import {AppstoreAddOutlined, MenuOutlined,} from '@ant-design/icons';
import {Link} from "react-router-dom";

const {Header: AntHeader} = Layout;


const Header = () => {
    return (
        <AntHeader className="header">
            <div className="logo">
                <Link to="/">
                    <AppstoreAddOutlined style={{fontSize: '24px', color: '#fff'}}/>
                    <span style={{color: '#fff', marginLeft: '10px'}}>Network Monitor</span>
                </Link>
            </div>
            <div style={{float: 'right', display: 'flex', alignItems: 'center'}}>
                <Button
                    icon={<MenuOutlined/>}
                    className="drawer-toggle"
                    style={{display: 'none'}} // Hidden by default
                />
            </div>
        </AntHeader>
    );
};

export default Header;
