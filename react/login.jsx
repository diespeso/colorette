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

        let token = await axios.post('/api/user', {
            username: document.getElementById('user').value,
            pass: document.getElementById('pass').value
        })
        .then( response => response.data)

        document.cookie = `jwt=${token}`
    }

    render() {
        return (<div>
            <label>Usuario</label>
            <input type="text" id="user"></input>
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