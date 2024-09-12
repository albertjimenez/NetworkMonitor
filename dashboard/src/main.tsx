import  {StrictMode} from 'react'
import {createRoot} from 'react-dom/client'
import App from './App.tsx'
import './index.css';
import 'antd/dist/reset.css';
import Footer from "./components/Footer.tsx";
import {BrowserRouter} from "react-router-dom";
import {MetricsProvider} from "./context/MetricsContext.tsx";


createRoot(document.getElementById('root')!).render(
    <StrictMode>
        <BrowserRouter>
            <MetricsProvider>
                <App/>
            </MetricsProvider>
        </BrowserRouter>
        <Footer/>
    </StrictMode>,
)
