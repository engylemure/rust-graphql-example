const casual = require('casual')
const fetch = require('node-fetch')

const usersPromise = []
const number = 100;

for(let i = 0; i < 100; i++) {
	let user = `user: { name: "${casual.name}", email: "${casual.email}", password: "${casual.password}" }`;
	let body = {
                        operationName: null,
                        variables: {},
                        query: `mutation { register( ${user} ) { value } }`
                };
	let config = { 
		method: 'POST',
		body: JSON.stringify(body)
	}
	let promise = fetch('http://127.0.0.1:8080', config);
	usersPromise.push(promise);
}

Promise.all(usersPromise).then(() => console.log('Finished'))
