let big_mac = require('../');

console.log(big_mac);

console.log(`Your mac address is ${big_mac.get_mac()}`);

console.log(`Your loopback mac address is ${big_mac.get_mac_by_name('lo0')}`);
