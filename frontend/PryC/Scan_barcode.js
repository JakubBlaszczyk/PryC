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
          ref={(cam) => {
            this.camera = cam;
          }}
          style={styles.container}
          aspect={Camera.constants.Aspect.fill}
          onBarCodeRead={this._onBarCodeRead}>
        </RNCamera>
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