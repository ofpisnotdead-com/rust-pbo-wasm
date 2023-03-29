async function run_wasm() {
  var myWorker = new Worker('./worker.js');

  document.getElementById("file_picker").addEventListener(
    "change",
    function() {
      let el = document.getElementById('content');
      el.innerHTML = '<h2><center>loading ...<br />it can take a while for big files</center></h2>';

      let file = this.files[0];
      myWorker.postMessage({ file: file });
      myWorker.onmessage = function(e) {
        let el = document.getElementById('content');
        el.innerHTML = '';

        if(e.data.headers.size > 0) {
          let info = document.createElement('h2');
          info.innerText = `headers in ${file.name}`;
          let text = document.createElement('p');
          let headersText = '';
          e.data.headers.forEach((v,k) => headersText = headersText + `${k}: ${v}\n`)
          text.innerText = headersText;
          el.appendChild(info);
          el.appendChild(text);
        }

        let info = document.createElement('h2');
        info.innerText = `files in ${file.name}:`;
        el.appendChild(info);

        let list = document.createElement('ul');
        e.data.entries.forEach(pboEntry => {
          let entry = document.createElement('li');
          let metadata = document.createElement('ul');

          entry.innerText = pboEntry.filename;

          ['packaging_method', 'original_size', 'reserved', 'timestamp', 'data_size'].forEach(key => {
            let item = document.createElement('li');
            item.innerText = `${key}: ${pboEntry[key]}`;
            metadata.appendChild(item);
          })
          entry.appendChild(metadata);
          list.appendChild(entry);
        });
        el.appendChild(list);
      };
    },
    false
  );
}

run_wasm();
