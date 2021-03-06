import React, {useContext, useEffect, useState} from 'react';
import Container from 'react-bootstrap/Container';
import ModalTitle from 'react-bootstrap/ModalTitle';
import {UserServiceContext} from "../Router";
import ChangeEmail from "./ChangeEmail";
import ChangePassword from "./ChangePassword";
import User from "../../data/user/User";
import LinkDiscord from "./LinkDiscord";

const Account = () => {
    const userService = useContext(UserServiceContext);
    const [currentUser, setCurrentUser] = useState<User | undefined>();

    const effect = () => {
        userService.getCurrentUser().then(setCurrentUser).catch(err => {
            switch (err.message) {
                case 'token-expired':
                    userService.forceUpdate().then(() => effect());
                    break;
            }
        })
        userService.onUserChange(user => setCurrentUser(user))
    }
    // eslint-disable-next-line
    useEffect(effect, [])
    return (
        <Container className={'text-center'}>
            <ModalTitle>Account von {currentUser?.email}</ModalTitle>
            <br/>
            <hr/>
            <ChangeEmail/>
            <hr/>
            <ChangePassword/>
            <hr/>
            <LinkDiscord/>
        </Container>
    );
};

export default Account;