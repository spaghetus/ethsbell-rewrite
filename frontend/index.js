let notification

async function update() {
	let messageTimer = setTimeout(() => {
		getel("message").innerHTML = "(We refresh our copy of the calendars during the first request of the day, so your request might be delayed if you're the first client of the day. Sorry for any inconvenience.)"
	}, 1000);
	let data = await get();
	clearTimeout(messageTimer);
	getel("message").innerHTML = "";
	let sets = ['prev', 'curr', 'next']
	for (let i = 0; i < data.length; ++i) {
		let parent = getel(`${sets[i]}_parent`);
		if (data[i]) {
			put(data[i], sets[i], i === 1)
			if (parent) {
				parent.style.display = "block";
			}
		} else {
			if (parent) {
				parent.style.display = "none";
			} else {
				put({friendly_name: "No School"}, sets[i])
			}
		}
	}
	if (data[1].end) {
		let elapsed = data[1].end.getTime() - (new Date(Date.now())).getTime()
		setTimeout(update, elapsed)
	}
}

async function get() {
	let data = await (await fetch("/api/v1/today/now/near")).json()
	for (let i of data) {
		if (!i) {continue}
		let start_date = new Date(Date.now());
		let end_date = new Date(Date.now());
		let start_split = i.start.split(":")
		let end_split = i.end.split(":")
		start_date.setHours(start_split[0])
		start_date.setMinutes(start_split[1])
		start_date.setSeconds(0)
		end_date.setHours(end_split[0])
		end_date.setMinutes(end_split[1])
		end_date.setSeconds(0)
		i.start = start_date
		i.end = end_date
	}
	console.log(data);
	return data
}

function put(period, element_set, notify) {
	let schedule = false;
	try {
		schedule = JSON.parse(localStorage.getItem('schedule'));
	} catch {}
	let friendly_name = period.friendly_name;
	let url;
	if (period?.kind?.Class != undefined && schedule && schedule[period.kind.Class]) {
		let period_def = schedule[period.kind.Class]
		friendly_name = period_def.name	
		url = period_def.url
	}
	let start = getel(`${element_set}_start`);
	let end = getel(`${element_set}_end`);
	let name = getel(`${element_set}_name`);
	let time_parent = getel(`${element_set}_time_parent`);
	if (period.start && period.end) {
		time_parent.style.display = "block"
	} else {
		time_parent.style.display = "none"
	}
	if (start && period.start) {
		start.innerHTML = ` at ${period.start.toLocaleTimeString()}`
	}
	if (end && period.end) {
		end.innerHTML = ` until ${period.end.toLocaleTimeString()}`
	}
	if (name) {
		name.innerHTML = `${url ? `<a href=${url}>` : ''}${friendly_name}${url ? `</a>` : ''}`
	}
	if (notify) {
		if (notification !== undefined) {
			if (notification.close) {
				notification.close()
			}
			notification = new Notification(`New period ${friendly_name}`);
		} else {
			notification = true
		}
	}
}

function getel(id) {
	let selector = `#${id}`
	return document.querySelector(selector)
}

getel("enable_notifications").addEventListener('click', async () => {
  await Notification.requestPermission();
})

update()