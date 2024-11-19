let cache = await caches.open('my_cache');

fetch('http://web.simmons.edu/').then((response) => {
    cache.put('http://web.simmons.edu/', response);
});

cache.match('http://web.simmons.edu/').then((response) => {
    console.log('Got response from cache!');
});