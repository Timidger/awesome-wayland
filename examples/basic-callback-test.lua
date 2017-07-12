--- Tests basic callback functions for all object types.
print("Starting basic callback test")

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


print("Basic callback test complete")
