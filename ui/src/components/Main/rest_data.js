//////////
// REST //
//////////

import * as store from '../local_storage';

function fetch_feeds() {
  var url = `/api/feeds${store.access_token_str()}`;
  var req = make_req(url, 'GET');
  fetch(req)
    .then(resp => resp.json())
    .then(data => {
      if (data) { this.setState({ feeds_data: data }) }
    })
    .catch(error => console.log("fetch feeds error: ", error));
}

function fetch_items(id) {
  var url = `/api/items/${id}${store.access_token_str()}`;
  if (this.state.last_date) {
    url = `${url}&updated=${this.state.last_date}`;
  }
  var req = make_req(url, 'GET');
  fetch(req)
    .then(resp => resp.json())
    .then(data => {
      if (data) {
        let last_date = data[data.length - 1].published_at;
        if (this.state.last_date) {
          this.setState((prevState, _props) => ({
            last_date: last_date,
            items_data: prevState.items_data.concat(data)
          }));
        } else {
          this.setState({
            last_date: last_date,
            items_data: data
          });
        }
      }
    })
    .catch(error => console.log("fetch items error: ", error));
}

function fetch_item(id) {
  var url = `/api/item/${id}${store.access_token_str()}`;
  var req = make_req(url, 'GET');
  fetch(req)
    .then(resp => resp.json())
    .then(data => {
      if (data) {
        this.setState({ selected_item: data, selected_item_id: id });
      }
    })
    .catch(error => console.log("fetch item error: ", error));
}


function make_req(url, verb) {
  var headers = new Headers({
    'Content-Type': 'application/json',
  });
  return new Request(url, {
    method: verb,
    headers: headers
  });
}



export { fetch_feeds, fetch_item, fetch_items }
