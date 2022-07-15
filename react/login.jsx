import axios from 'axios'
class Login extends React.Component {
    constructor(props) {
        super(props)

        this.handleSubmit = this.handleSubmit.bind(this)
    }

    async handleSubmit() {
        console.log("clicked")
        /*var f = await fetch('http://localhost:8000/api/user')
        .then(response => response.json())
        .then(data => console.log(data))*/
        //this is for sign up
        
        let token = await axios.post('/api/session', {
            email: document.getElementById('email').value,
            pass: document.getElementById('pass').value
        })
        .then( response => {document.cookie = `jwt=${response.data.jwt};path=/`})

        //document.cookie = `jwt=${token.jwt};path=/`
    }

    render() {
        return (<div>
            <label>E-mail</label>
            <input type="text" id="email"></input>
            <br/>
            <label>Contraseña</label>
            <input type="password" id="pass"></input>
            <br/>
            <button onClick={this.handleSubmit}>Log in</button>
        </div>)
    }
}

const container = document.getElementById('login-hook')
const root = ReactDOM.createRoot(container)
root.render(<Login />)