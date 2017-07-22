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

-- Test drawin functions
drawin.add_signal()
drawin.connect_signal()
drawin.disconnect_signal()
drawin.emit_signal()
drawin.instances()
drawin.set_index_miss_handler()
drawin.set_newindex_miss_handler()
drawin.__call()
drawin.geometry()

-- Test drawin meta functions
-- TODO Lib doesn't add meta functions yet
--drawin.__metatable.__tostring()
--drawin.__metatable.connect_signal()
--drawin.__metatable.disconnect_signal()
--drawin.__metatable.__index()
--drawin.__metatable.__newindex()


-- Test keygrabber functions
keygrabber.run()
keygrabber.stop()
keygrabber.isrunning()
keygrabber.__index()
keygrabber.__newindex()

-- Test mousegrabber fuctions
mousegrabber.run()
mousegrabber.stop()
mousegrabber.isrunning()
mousegrabber.__index()
mousegrabber.__newindex()


-- Test mouse functions
mouse.__index()
mouse.__newindex()
mouse.coords()
mouse.object_under_pointer()
mouse.set_index_miss_handler()
mouse.set_newindex_miss_handler()


-- Test root functions
root.buttons()
root.keys()
root.cursor()
root.fake_input()
root.drawins()
root.wallpaper()
root.size()
root.size_mm()
root.tags()
root.__index()
root.__newindex()


-- Test screen functions
screen.add_signal()
screen.connect_signal()
screen.disconnect_signal()
screen.emit_signal()
screen.instances()
screen.set_index_miss_handler()
screen.set_newindex_miss_handler()
screen.count()
screen.__index()
screen.__newindex()
screen.__call()
screen.fake_add()
screen.fake_remove()
screen.fake_resize()
screen.swap()

-- Test screen meta functions
-- TODO Lib doesn't add meta functions yet
--screen.__metatable.__tostring()
--screen.__metatable.connect_signal()
--screen.__metatable.disconnect_signal()
--screen.__metatable.__index()
--screen.__metatable.__newindex()

-- Test tag functions
tag.add_signal()
tag.connect_signal()
tag.disconnect_signal()
tag.emit_signal()
tag.instances()
tag.set_index_miss_handler()
tag.set_newindex_miss_handler()
tag.__call()

-- Test tag meta functions
-- TODO Lib doesn't add meta functions yet
--tag.__metatable.__tostring()
--tag.__metatable.connect_signal()
--tag.__metatable.disconnect_signal()
--tag.__metatable.__index()
--tag.__metatable.__newindex()
--tag.__metatable.clients()

print("Basic callback test complete")
