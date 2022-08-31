import os

base_address = 0x80400000
step = 0x20000
linker = 'src/linker.ld'

app_id = 0
apps = os.listdir('src/bin')
apps.sort()
with open(linker, 'w+') as f:
  for app in apps:
      app = app[:app.find('.')]
      lines = []
      lines_before = []
      for line in f.readlines():
          lines_before.append(line)
          line = line.replace(hex(base_address), hex(base_address+step*app_id))
          lines.append(line)
      f.writelines(lines)
      os.system('cargo build --bin %s --release' % app)
      print('[build.py] application %s start with address %s' %(app, hex(base_address+step*app_id)))
      f.writelines(lines_before)
      app_id = app_id + 1
