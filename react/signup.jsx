import axios from 'axios'

class SignUp extends React.Component {
    constructor(props) {
        super(props)

        this.handleSubmit = this.handleSubmit.bind(this)
    }

    /**
     * Will sign up but wont log in, gotta log in with other page
     * just for now, will be better later i think
     */
    async handleSubmit() {
        let res = await axios.post('/api/user', {
            email: document.getElementById('email').value,
            pass: document.getElementById('pass').value
        })
            .then(response => {console.log('usuario creado ', document.getElementById('email').value)})
            .catch(err => console.log('signup.js:', err.response))
    }

    render() {
        return (<div>
            <label>E-mail</label>
            <input type="text" id="email"></input>
            <br/>
            <label>Contraseña</label>
            <input type="password" id="pass"></input>
            <br/>
            <button onClick={this.handleSubmit}>Sign Up</button>
        </div>)
    }
}

export default SignUp

const container = document.getElementById("signup-hook")
const root = ReactDOM.createRoot(container)
root.render(<SignUp/>)