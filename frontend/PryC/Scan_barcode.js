import React, { Component } from 'react';
import {
  AppRegistry,
  Dimensions,
  StyleSheet,
  Text,
  TouchableHighlight,
  View
} from 'react-native';
import { RNCamera } from 'react-native-camera';

class Scan_barcode extends Component {

  constructor(props) {
    super(props);
    this.state = {
      showCamera: true,
    };
  }

  renderCamera = () => {
    if(this.state.showCamera) {
      return (
        <RNCamera
          ref={ref => {
            this.camera = ref;
          }}
          style={styles.preview}
          type={RNCamera.Constants.Type.back}
          flashMode={RNCamera.Constants.FlashMode.on}
          androidCameraPermissionOptions={{
            title: 'Permission to use camera',
            message: 'We need your permission to use your camera',
            buttonPositive: 'Ok',
            buttonNegative: 'Cancel',
          }}
          androidRecordAudioPermissionOptions={{
            title: 'Permission to use audio recording',
            message: 'We need your permission to use your audio',
            buttonPositive: 'Ok',
            buttonNegative: 'Cancel',
          }}
          onGoogleVisionBarcodesDetected={this._onBarCodeRead}
        />
      );
    } else {
      return (
        <View></View>
      );
    }
  }

  render() {
    return (
      this.renderCamera()
    );
  }

  _onBarCodeRead = (e) => {
    this.setState({showCamera: false});
    alert("Barcode Found!",
          "Type: " + e.type + "\nData: " + e.data);
  }
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: "center",
    alignItems: "center",
    backgroundColor: "transparent",
  },
});

export default Scan_barcode;