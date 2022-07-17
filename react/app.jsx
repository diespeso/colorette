import Clock from './clock.js'
import LikeButton from './like_button.js'
import Stopper from './stopper.js'

const container = document.getElementById('like_button_container')
const root = ReactDOM.createRoot(container)
root.render(<Stopper />)