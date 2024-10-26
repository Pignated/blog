import { useState } from 'react';


import { Col, Button, Row, Container, Card, Form } from 'react-bootstrap'

function Signup() {
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
    const [email, setEmail] = useState("");
    function updatePassword(e) {
        setPassword(e.target.value)
    }
    function updateUsername(e){
        setUsername(e.target.value)
    }
    function updateEmail(e) {
        setEmail(e.target.value)
    }
    function sendSignin() {
        fetch('/api/auth/signup', 
            {
                method: "POST",
                body: JSON.stringify({username:username, password:password, email:email}),
                headers: {
                    "Content-Type": "application/json",
                },
            }
        ).then(res => console.log(res))
  
    }
    return (
        <Container>
            <Row className="vh-100 d-flex justify-content-center align-items-center">
                <Col md={8} lg={6} xs={12}>
                <div className="pt-auto"></div>
                    <Card className="shadow border-3 border-gamer border-top pt-auto rounded-5 " >
            <Card.Body>
                <h1 className="h1">Please enter your login and password!</h1>
            <Form >
                <Form.Group  controlId='formBasicUsername'> 
                    <Form.Label>Username</Form.Label>
                    <Form.Control type='username' placeholder='Enter Username' onChange={updateUsername}/>
                    <Form.Text>We don't share this with anyone</Form.Text>
                </Form.Group>
                <Form.Group controlId='formBasicEmail'> 
                    <Form.Label >Email</Form.Label>
                    <Form.Control type='email' placeholder='Enter Email' onChange={updateEmail}/>
                </Form.Group>
                <Form.Group controlId='formBasicPassword'> 
                    <Form.Label>Password</Form.Label>
                    <Form.Control type='password' placeholder='Enter Password' onChange={updatePassword}/>
                </Form.Group>
                <div style={{alignContent:'center'}}>
                <Button className='mt-1 mx-auto' onClick={sendSignin}>Hello</Button>
                </div>
                <Form.Label className='text-danger ms-3 mt-3 '>Accounts not currently implemented</Form.Label>
            </Form>


            </Card.Body>
          </Card>
            </Col>
            </Row>
        </Container>
    )
}

export default Signup;