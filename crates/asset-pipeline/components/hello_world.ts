//define a class extending HTMLElement
class HelloWorld extends HTMLElement {
    connectedCallback () {
      this.innerHTML = 'Hello, World! Hello RUST!'
    }
}

//register the new custom element
customElements.define( 'hello-world', HelloWorld )
