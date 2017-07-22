--- Tests basic callback functions for all object types.
print("Starting basic callback test")

-- TODO Test properties

-- Test awesome functions
awesome.quit()
awesome.exec()
awesome.spawn()
awesome.restart()
awesome.connect_signal()
awesome.disconnect_signal()
awesome.emit_signal()
awesome.systray()
awesome.load_image()
awesome.set_preferred_icon_size()
awesome.register_xproperty()
awesome.set_xproperty()
awesome.get_xproperty()
awesome.__index()
awesome.__newindex()
awesome.xkb_set_layout_group()
awesome.xkb_get_layout_group()
awesome.xkb_get_group_names()
awesome.xrdb_get_value()
awesome.kill()
awesome.sync()

-- Test button functions
button.add_signal()
button.connect_signal()
button.disconnect_signal()
button.emit_signal()
button.instances()
button.set_index_miss_handler()
button.set_newindex_miss_handler()
button.__call()

-- Test button properties
button.button()
button.modifiers()

-- Test button meta functions
-- TODO Lib doesn't add meta functions yet
--button.__metatable.__tostring()
--button.__metatable.__index()
--button.__metatable.__newindex()
--button.__metatable.disconnect_signal()
--button.__metatable.connect_signal()

-- Test client functions
client.add_signal()
client.connect_signal()
client.disconnect_signal()
client.emit_signal()
client.instances()
client.set_index_miss_handler()
client.set_newindex_miss_handler()
client.__call()
client.__get()
client.__index()
client.__newindex()
client.keys()
client.isvisible()
client.geometry()
client.apply_size_hints()
client.tags()
client.kill()
client.swap()
client.raise()
client.lower()
client.unmanange()
client.titlebar_top()
client.titlebar_right()
client.titlebar_bottom()
client.titlebar_left()
client.get_icon()

-- Test client meta functions
-- TODO Lib doesn't add meta functions yet
--client.__metatable.__tostring()
--client.__metatable.connect_signal()
--client.__metatable.disconnect_signal()
--client.__metatable.__index()
--client.__metatable.__newindex()

print("Basic callback test complete")
