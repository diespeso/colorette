import axios from 'axios'
//https://api.rocket.rs/v0.5-rc/rocket/fs/enum.TempFile.html

class ImageUploader extends React.Component {
    constructor(props) {
        super(props)

        this.handleSubmit = this.handleSubmit.bind(this)
    }

    async handleSubmit() {
        let form = new FormData();
        form.append("name", document.getElementById("name").value)
        form.append("file", document.getElementById("file").files[0])
        console.log('form', form)
        let res = await axios.post('/api/user_images', form, {
            headers: {
                'Content-Type': 'multipart/form-data'
            }
        })
    }

    render() {
        return (<div>
            <label>Image</label>
            <input type="file" id="file" accept="image/jpeg"></input>
            <br/>
            <label>Name</label>
            <input type="text" id="name"></input>
            <br/>
            <button onClick={this.handleSubmit}>Upload</button>
        </div>)
    }
}

export default ImageUploader

const container = document.getElementById('root')
const root = ReactDOM.createRoot(container)
root.render(<ImageUploader />)