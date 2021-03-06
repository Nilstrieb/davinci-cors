import React, {useContext, useEffect, useState} from 'react';
import Alert from 'react-bootstrap/Alert';
import Button from 'react-bootstrap/Button';
import Col from 'react-bootstrap/Col';
import Container from 'react-bootstrap/Container';
import Form from 'react-bootstrap/Form';
import FormGroup from 'react-bootstrap/FormGroup';
import FormControl from "react-bootstrap/FormControl";
import FormLabel from 'react-bootstrap/FormLabel';
import Row from 'react-bootstrap/Row';
import {UserServiceContext} from "../Router";
import * as Yup from 'yup';
import {useFormik} from "formik";
import User from "../../data/user/User";

const ChangeEmail = () => {
    const userService = useContext(UserServiceContext);
    const [currentUser, setCurrentUser] = useState<User>();

    const effect = () => {
        userService.getCurrentUser().then(setCurrentUser).catch((err) => {
            if (err.message === 'token-expired')
                userService.forceUpdate().then(() => effect())
        });
    }

    // eslint-disable-next-line
    useEffect(effect, [])

    const handleSubmit = ({email}: { email: string }) => {
        userService.changeEmail(email).catch(err => {
            switch (err.message) {
                case 'token-expired':
                    userService.forceUpdate().then(() => handleSubmit({email}));
            }
        });
    }


    const formik = useFormik({
        onSubmit: handleSubmit,
        validateOnChange: false,
        validateOnBlur: true,
        initialValues: {
            'email': currentUser?.email || ''
        },
        validationSchema: Yup.object().shape({
            'email': Yup.string()
                .email('Bitte geben Sie eine valide E-Mail ein')
                .max(50, 'Die E-Mail darf maximal 50 Zeichen lang sein')
                .required('Das E-Mail Feld darf nicht leer sein')
        })
    })
    return (
        <Container>
            <br/>
            <h3 className={'modal-title'}>E-Mail ändern</h3>
            <br/>
            <Form onSubmit={e => {
                e.preventDefault();
                formik.handleSubmit(e);
            }} inline>
                <Row>
                    <Col>
                        <FormGroup>
                            <FormLabel>E-Mail Adresse</FormLabel>
                            <FormControl name={'email'} placeholder={'E-Mail Adresse ändern'}
                                         onChange={formik.handleChange} isInvalid={!!formik.errors.email}/>
                            <Alert variant={'danger'} show={!!formik.errors.email}></Alert>
                        </FormGroup>
                        <Alert variant={'danger'} show={!!formik.errors.email}>{formik.errors.email}</Alert>
                    </Col>
                    <Col>
                        <br/>
                        <Button type={'submit'}>E-Mail ändern</Button>
                    </Col>
                </Row>
            </Form>
            <br/>
        </Container>
    );
};

export default ChangeEmail;