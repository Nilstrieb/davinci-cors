import React, {useContext, useEffect, useState} from 'react';
import Tab from 'react-bootstrap/Tab';
import Tabs from 'react-bootstrap/Tabs';
import Container from 'react-bootstrap/Container';
import {Redirect, Route, Switch, useHistory, useParams} from 'react-router-dom';
import ClassInfo from "./ClassInfo";
import Timetable from "./timetable/Timetable";
import Calendar from "./calendar/Calendar";
import {UserServiceContext} from "../../Router";
import Class from "../../../data/class/Class";
import WieLangeNoch from "./wielangenoch/WieLangeNoch";
import AdminPanel from "./adminPanel/AdminPanel";
import Settings from "../../settings/Settings";

const CurrentClass = React.createContext<Class | undefined>(undefined)

const ClassView = () => {
    const {id} = useParams<{ id: string }>();
    const [currentClass, setCurrentClass] = useState<Class>();
    const [selectedSite, setSelectedSite] = useState<string>('info');
    const [isAdmin, setIsAdmin] = useState(false);
    const history = useHistory();
    const userService = useContext(UserServiceContext);
    useEffect(() => {
        if (id) {
            userService.getClass(id).then(setCurrentClass);
        }
    }, [id, userService])

    useEffect(() => {
        if (currentClass) {
            setIsAdmin(userService.isAdmin(currentClass))
        }
        // eslint-disable-next-line
    }, [currentClass])

    const onTabSelect = (key: string | null) => {
        if (key) {
            setSelectedSite(key);
        }
    }
    useEffect(() => {
        if (id && currentClass) {
            history.push(`/class/${id}/${selectedSite}`);
        }
    }, [history, id, selectedSite, currentClass])

    return (
        <Container fluid>

            <CurrentClass.Provider value={currentClass}>
                {
                    currentClass && (
                        <>
                            <Tabs id={'classview-tab'} className={'mb-3'} activeKey={selectedSite}
                                  onSelect={onTabSelect} sm={8} transition={false}>
                                <Tab eventKey={'info'} title={'Info'}/>
                                <Tab eventKey={"timetable"} title={'Stundenplan'}/>
                                <Tab eventKey={'calendar'} title={'Kalender'}/>
                                <Tab title={'Wie Lange Noch'} eventKey={'wielangenoch'}/>
                                <Tab title={'Einstellungen'} eventKey={'settings'}/>
                                {
                                    isAdmin && <Tab title={'Admin'} eventKey={'admin'}/>
                                }
                            </Tabs>

                            <Switch>
                                <Route path={'/class/:id/info'} component={ClassInfo}/>
                                <Route path={'/class/:id/calendar'} component={Calendar}/>
                                <Route path={'/class/:id/timetable'} component={Timetable}/>
                                <Route path={'/class/:id/wielangenoch'} component={WieLangeNoch}/>
                                <Route path={'/class/:id/settings'} component={Settings}/>
                                {
                                    isAdmin &&
                                    <Route path={'/class/:id/admin'} component={AdminPanel}/>
                                }
                            </Switch>

                            <Redirect exact from={'/class/:id/'} to={`/class/${id}/info`}/>
                        </>
                    )
                }
            </CurrentClass.Provider>
        </Container>
    );
};


export default ClassView;
export {CurrentClass}