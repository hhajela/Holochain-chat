var holochain_connection = holochainclient.connect();

function hello() {
	  holochain_connection.then(({callZome, close}) => {
		      callZome(
      'test-instance',
      'hello',
      'hello_holo',
			          )({args: {}}).then(result => show_output(result,'output'));
  });
}

function show_output(result, id) {

	var span = document.getElementById(id);

	var output = JSON.parse(result);

	if (output.Ok) {

		if (id === 'name_output') {
			span.textContent = ' ' + output.Ok.name;
		} else {
			span.textContent = ' ' + output.Ok;
		}
	} else {
		alert(output.Err.Internal);
	}

	
}

function create_post() {

	//get post content from text area
	const post_txt = document.getElementById('post').value;
	
	holochain_connection.then(({callZome,close}) => {
		callZome('test-instance','hello','create_post')({message: post_txt, timestamp: Date.now()}).then(result => show_output(result,'address_output'));
	});
}

function retrieve_posts() {

	const addr = document.getElementById('address').value;

	holochain_connection.then(({callZome, close}) => {
		callZome('test-instance','hello','retrieve_posts')({agent_addr: addr}).then(result => display_posts(result));
	});

}

function display_posts(post_list) {

	var list = document.getElementById('posts');
	list.innerHTML = "";

	var output = JSON.parse(post_list);

	if (output.Ok) {
		var posts = output.Ok.sort( (a,b) => a.timestamp - b.timestamp );

		for (post of posts) {
			var node = document.createElement("LI");
			var text = document.createTextNode(post.message);
			node.appendChild(text);
			list.appendChild(node);
		}
	} else {
		alert(output.Err.Internal);
	}
}

function get_agent_id() {

	holochain_connection.then( ({callZome,close}) => {
		callZome('test-instance','hello','get_agent_id')({args: {}}).then(result => show_output(result, 'agent_id'));
	});
}

