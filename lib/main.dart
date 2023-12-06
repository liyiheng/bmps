import 'dart:io';
import 'dart:ui';

import 'package:bmps/messages/bg.pb.dart' as bg;
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:image_picker/image_picker.dart';
import 'package:path/path.dart' as path;
import 'package:rinf/rinf.dart';

void main() async {
  await Rinf.ensureInitialized();
  runApp(const ProviderScope(child: MyApp()));
}

class ValNotifier<T> extends ChangeNotifier {
  late T val;

  ValNotifier(this.val);

  void set(T val) {
    if (this.val == val) {
      return;
    }
    this.val = val;
    notifyListeners();
  }
}

final orientationProvider = ChangeNotifierProvider<ValNotifier<Orientation>>(
    (ref) => ValNotifier<Orientation>(Orientation.landscape));

class MyApp extends ConsumerWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context, WidgetRef reg) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const MyHomePage(title: 'Flutter Demo Home Page'),
    );
  }
}

class MyHomePage extends ConsumerStatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  ConsumerState<MyHomePage> createState() => _MyHomePageState();
}

enum Orientation {
  landscape,
  portrait;
}

typedef OnPickImageCallback = void Function(
    double? maxWidth, double? maxHeight, int? quality);

class _MyHomePageState extends ConsumerState<MyHomePage> {
  final ImagePicker _picker = ImagePicker();
  String _originPath = '';
  String _destPath = '';

  @override
  Widget build(BuildContext context) {
    var orien = ref.watch(orientationProvider).val;
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text(widget.title),
      ),
      body: Column(
        mainAxisAlignment: MainAxisAlignment.start,
        children: <Widget>[
          if (_originPath.isNotEmpty) ...[
            SizedBox(
              height: 200,
              child: ImageFiltered(
                imageFilter: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
                child: Image.file(
                  File(_originPath),
                  fit: BoxFit.scaleDown,
                  height: 200,
                ),
              ),
            ),
          ],
          ListTile(
            title: const Text('横屏'),
            trailing: const Icon(Icons.crop_landscape),
            leading: Radio<Orientation>(
              groupValue: orien,
              value: Orientation.landscape,
              onChanged: (v) {
                if (v != null) {
                  ref.read(orientationProvider).set(v);
                }
              },
            ),
          ),
          ListTile(
            title: const Text('竖屏'),
            trailing: const Icon(Icons.crop_portrait),
            leading: Radio<Orientation>(
              groupValue: orien,
              value: Orientation.portrait,
              onChanged: (v) {
                if (v != null) {
                  ref.read(orientationProvider).set(v);
                }
              },
            ),
          ),
          FloatingActionButton(
            onPressed: () async {
              XFile? file = await _picker.pickImage(
                  source: ImageSource.gallery, maxHeight: 1000, maxWidth: 1000);
              print(file);
              if (file == null) {
                return;
              }
              setState(() {
                _originPath = file.path;
              });
            },
            heroTag: 'image0',
            tooltip: 'Pick Image from gallery',
            child: const Icon(Icons.photo),
          ),
          Text(_originPath),
          if (_originPath.isNotEmpty) ...[
            SizedBox(
              height: 100,
              child: FractionallySizedBox(
                heightFactor: 0.3,
                widthFactor: 1,
                child: AspectRatio(
                  aspectRatio: 16 / 9,
                  child: Stack(
                    children: [
                      Padding(
                        padding: const EdgeInsets.all(54),
                        child: Image.file(
                          File(_originPath),
                          fit: BoxFit.scaleDown,
                          height: 300,
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ),
          ],
          ElevatedButton(
              onPressed: () async {
                if (_originPath.isEmpty) {
                  print('path is empty, skip');
                  return;
                }
                var (width, height) = (1920, 1080);
                if (ref.read(orientationProvider).val == Orientation.portrait) {
                  (width, height) = (height, width);
                }
                print('width: $width, height:$height');
                var fname = path.basename(_originPath);
                var dir = path.dirname(_originPath);
                var dest = path.join(dir, 'bmps_$fname');
                var req = bg.GenRequest(
                    source: _originPath,
                    dest: dest,
                    width: width,
                    height: height,
                    blurRadius: 50,
                    roundRadius: 30,
                    shadow: 30,
                    padding: 0.1);
                var rreq = RustRequest(
                    resource: bg.ID,
                    operation: RustOperation.Read,
                    message: req.writeToBuffer());
                final rresp = await requestToRust(rreq);
                var resp = bg.GenResponse.fromBuffer(rresp.message!);
                print(resp);
                if (resp.code == 0) {
                  setState(() {
                    _destPath = dest;
                  });
                }
              },
              child: Text("生成")),
          if (_destPath.isNotEmpty) ...[
            Expanded(
              child: Image.file(
                File(_destPath),
                fit: BoxFit.scaleDown,
              ),
            ),
          ]
        ],
      ),
    );
  }
}
// padding 0.15
// blur 50
// round 45
// shadow 40
