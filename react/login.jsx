import axios from 'axios'
class Login extends React.Component {
    constructor(props) {
        super(props)

        this.handleSubmit = this.handleSubmit.bind(this)
    }

    async handleSubmit() {
        
        let token = await axios.post('/api/session', {
            email: document.getElementById('email').value,
            pass: document.getElementById('pass').value
        })
        .then( response => {document.cookie = `jwt=${response.data.jwt};path=/`})
        .catch( err => { console.log('app.js error:', err.response)})

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

export default Login

const container = document.getElementById('login-hook')
const root = ReactDOM.createRoot(container)
root.render(<Login />)