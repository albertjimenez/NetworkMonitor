import React from "react";
import {getCurrentYear} from "../utils/dateUtils.ts";
import {Layout} from "antd";


const Footer: React.FC = () => (
    <Layout.Footer style={{textAlign: 'center'}}>
        &copy; {getCurrentYear()} Network Monitor. All rights reserved.
    </Layout.Footer>
);
export default Footer;
