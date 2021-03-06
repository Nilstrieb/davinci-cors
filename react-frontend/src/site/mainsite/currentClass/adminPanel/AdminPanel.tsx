import React from 'react';
import ChangeClassSettings from "./ChangeClassSettings";
import NewEvent from "./NewEvent";
import NewLesson from "./NewLesson";
import PendingMembers from "./PendingMembers";
import Members from "./edituser/Members";
import Tab from 'react-bootstrap/Tab';
import Tabs from 'react-bootstrap/Tabs';
import BannedMembers from "./BannedMembers";

const AdminPanel = () => {
    return (
        <Tabs id={'classview-tab'} className={'mb-3'}>
            <Tab eventKey={'changeClassSettings'} title={'Allgemeine Einstellungen'}><ChangeClassSettings/></Tab>
            <Tab eventKey={'newEvent'} title={'Eventverwaltung'}><NewEvent/></Tab>
            <Tab eventKey={'newLesson'} title={'Lektionverwaltung'}><NewLesson/></Tab>
            <Tab eventKey={'members'} title={'Mitgliederverwaltung'}><Members/></Tab>
            <Tab eventKey={'pendingMembers'} title={'Beitrittsanfragen'}><PendingMembers/></Tab>
            <Tab eventKey={'bans'} title={'Banns'}><BannedMembers/></Tab>
        </Tabs>
    );
};

export default AdminPanel;