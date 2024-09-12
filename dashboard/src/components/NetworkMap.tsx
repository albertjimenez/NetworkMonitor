import React from 'react';
import {Card} from 'antd';


const NetworkMap: React.FC = () => {
    return (
        <Card title="Network Map" bordered={false} style={{marginBottom: '24px'}}>
            <img
                src="../assets/react.svg"
                alt="Network Map"
                style={{width: '100%', height: 'auto'}}
            />
        </Card>
    );
};

export default NetworkMap;
